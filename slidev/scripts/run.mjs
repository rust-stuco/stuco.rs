import { spawn } from 'node:child_process'
import {
  accessSync,
  constants,
  rmSync,
  unwatchFile,
  watchFile,
} from 'node:fs'
import {
  cp,
  copyFile,
  mkdir,
  readFile,
  rm,
  symlink,
  writeFile,
} from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import path from 'node:path'

const slidevRoot = fileURLToPath(new URL('../', import.meta.url))
const repositoryRoot = path.resolve(slidevRoot, '..')
const sharedImages = path.join(repositoryRoot, 'lectures', 'images')
const runtimeRoot = path.join(slidevRoot, 'runtime')
const outputRoot = path.join(slidevRoot, 'dist')
const slidevCli = path.join(
  slidevRoot,
  'node_modules',
  '@slidev',
  'cli',
  'bin',
  'slidev.mjs',
)

const task = process.argv[2]
const lecture = process.argv[3]
const forwardedArgs = process.argv.slice(4)

if (!task || !lecture) {
  console.error(
    'Usage: node scripts/run.mjs <dev|build|export:light|export:dark> <NN_lecture>',
  )
  process.exitCode = 1
} else {
  await runTask(task, lecture, forwardedArgs)
}

async function runTask(taskName, lectureDirectory, extraArgs) {
  if (!/^\d{2}_[a-z0-9_]+$/.test(lectureDirectory)) {
    throw new Error(`Invalid lecture directory: ${lectureDirectory}`)
  }

  const slug = lectureDirectory.replace(/^\d{2}_/, '')
  const sourceLectureRoot = path.join(
    repositoryRoot,
    'lectures',
    lectureDirectory,
  )
  const sourceDeck = path.join(sourceLectureRoot, `${slug}.md`)
  const workspaceRoot = path.join(
    slidevRoot,
    '.slidev-work',
    lectureDirectory,
    `${taskName.replace(':', '-')}-${process.pid}`,
  )
  const lectureRoot = path.join(
    workspaceRoot,
    'lectures',
    lectureDirectory,
  )
  const setupRoot = path.join(lectureRoot, 'setup')
  const workspaceImages = path.join(workspaceRoot, 'lectures', 'images')
  const workspaceDeck = path.join(lectureRoot, `${slug}.md`)
  const taskOutputRoot = path.join(outputRoot, lectureDirectory)
  const siteOutputRoot = process.env.STUCO_SLIDEV_SITE_OUTPUT
    ? path.resolve(process.env.STUCO_SLIDEV_SITE_OUTPUT)
    : path.join(taskOutputRoot, 'deck')
  const pdfOutputRoot = process.env.STUCO_SLIDEV_PDF_OUTPUT
    ? path.resolve(process.env.STUCO_SLIDEV_PDF_OUTPUT)
    : taskOutputRoot
  const siteBase = `/lectures/${lectureDirectory}/deck/`
  const cleanupWorkspace = () =>
    rmSync(workspaceRoot, { recursive: true, force: true })

  accessSync(sourceDeck, constants.R_OK)

  await rm(workspaceRoot, { recursive: true, force: true })
  process.once('exit', cleanupWorkspace)
  try {
    await mkdir(path.dirname(lectureRoot), { recursive: true })
    await cp(sourceLectureRoot, lectureRoot, { recursive: true })
    await mkdir(setupRoot, { recursive: true })
    await symlink(
      sharedImages,
      workspaceImages,
      process.platform === 'win32' ? 'junction' : 'dir',
    )

    await Promise.all([
      copyFile(
        path.join(runtimeRoot, 'style.css'),
        path.join(lectureRoot, 'style.css'),
      ),
      copyFile(
        path.join(runtimeRoot, 'slide-bottom.vue'),
        path.join(lectureRoot, 'slide-bottom.vue'),
      ),
      copyFile(
        path.join(runtimeRoot, 'vite.config.ts'),
        path.join(lectureRoot, 'vite.config.ts'),
      ),
      copyFile(
        path.join(runtimeRoot, 'setup', 'shiki.ts'),
        path.join(setupRoot, 'shiki.ts'),
      ),
    ])

    // Static builds let Vite rewrite image paths. The development server and PDF exporter do not:
    // paths outside the lecture directory fall through to the Slidev app instead of serving the
    // image. Inline images in those temporary workspaces so both modes remain self-contained.
    if (taskName === 'dev' || taskName.startsWith('export')) {
      await inlineMarkdownImages(sourceDeck, workspaceDeck)
    }

    if (taskName === 'build') {
      await mkdir(siteOutputRoot, { recursive: true })
    } else if (taskName.startsWith('export')) {
      await mkdir(pdfOutputRoot, { recursive: true })
    }

    const taskArgs = {
      dev: [workspaceDeck],
      build: [
        'build',
        workspaceDeck,
        '--base',
        siteBase,
        '--out',
        siteOutputRoot,
        '--router-mode',
        'hash',
      ],
      'export:light': [
        'export',
        workspaceDeck,
        '--output',
        path.join(pdfOutputRoot, `${slug}-light.pdf`),
        '--with-toc',
        '--timeout',
        '60000',
        ...browserArgs(),
      ],
      'export:dark': [
        'export',
        workspaceDeck,
        '--output',
        path.join(pdfOutputRoot, `${slug}-dark.pdf`),
        '--dark',
        '--with-toc',
        '--timeout',
        '60000',
        ...browserArgs(),
      ],
    }[taskName]

    if (!taskArgs) {
      throw new Error(`Unknown Slidev task: ${taskName}`)
    }

    const stopSourceSync =
      taskName === 'dev'
        ? watchSourceDeck(sourceDeck, workspaceDeck)
        : () => {}
    try {
      const exitCode = await runSlidev([...taskArgs, ...extraArgs])
      if (exitCode !== 0) {
        process.exitCode = exitCode
      } else if (taskName === 'build') {
        await removeImagePreloads(siteOutputRoot)
      }
    } finally {
      stopSourceSync()
    }
  } finally {
    process.removeListener('exit', cleanupWorkspace)
    await rm(workspaceRoot, { recursive: true, force: true })
  }
}

async function inlineMarkdownImages(source, destination) {
  let markdown = await readFile(source, 'utf8')
  const htmlImagePattern = /<img\b[^>]*\bsrc=(["'])([^"']+)\1[^>]*>/g
  const markdownImagePattern =
    /!\[([^\]\n]*)\]\(\s*(?:<([^>\n]+)>|([^\s)\n]+))(?:\s+(?:"([^"\n]*)"|'([^'\n]*)'|\(([^)\n]*)\)))?\s*\)/g
  const imageSources = [
    ...[...markdown.matchAll(htmlImagePattern)].map((match) => match[2]),
    ...[...markdown.matchAll(markdownImagePattern)].map(
      (match) => match[2] ?? match[3],
    ),
  ]
  const localSources = [
    ...new Set(
      imageSources.filter(
        (imageSource) =>
          !imageSource.startsWith('data:') &&
          !imageSource.startsWith('http://') &&
          !imageSource.startsWith('https://'),
      ),
    ),
  ]

  const inlineImages = await Promise.all(
    localSources.map(async (imageSource) => {
      const imagePath = path.resolve(path.dirname(source), imageSource)
      const image = await readFile(imagePath)
      const mimeType = imageMimeType(imagePath)
      return [imageSource, `data:${mimeType};base64,${image.toString('base64')}`]
    }),
  )

  const dataUrls = new Map(inlineImages)
  markdown = markdown.replace(
    htmlImagePattern,
    (image, quote, imageSource) => {
      const dataUrl = dataUrls.get(imageSource)
      return dataUrl
        ? image.replace(
            `src=${quote}${imageSource}${quote}`,
            `src=${quote}${dataUrl}${quote}`,
          )
        : image
    },
  )
  markdown = markdown.replace(
    markdownImagePattern,
    (
      image,
      altText,
      enclosedSource,
      bareSource,
      doubleQuotedTitle,
      singleQuotedTitle,
      parenthesizedTitle,
    ) => {
      const imageSource = enclosedSource ?? bareSource
      const dataUrl = dataUrls.get(imageSource)
      if (!dataUrl) return image

      const title =
        doubleQuotedTitle ?? singleQuotedTitle ?? parenthesizedTitle
      const titleAttribute =
        title === undefined ? '' : ` title="${escapeHtmlAttribute(title)}"`
      return `<img style="min-height: 0; object-fit: contain;" src="${dataUrl}" alt="${escapeHtmlAttribute(altText)}"${titleAttribute}>`
    },
  )
  await writeFile(destination, markdown)
}

function escapeHtmlAttribute(value) {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('"', '&quot;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
}

async function removeImagePreloads(siteOutputRoot) {
  await Promise.all(
    ['index.html', '404.html'].map(async (fileName) => {
      const file = path.join(siteOutputRoot, fileName)
      const html = await readFile(file, 'utf8')
      const withoutImagePreloads = html.replace(
        /^\s*<link rel="preload" as="image"[^>]*>\r?\n/gm,
        '',
      )
      await writeFile(file, withoutImagePreloads)
    }),
  )
}

function imageMimeType(imagePath) {
  const mimeTypes = {
    '.avif': 'image/avif',
    '.gif': 'image/gif',
    '.jpeg': 'image/jpeg',
    '.jpg': 'image/jpeg',
    '.png': 'image/png',
    '.svg': 'image/svg+xml',
    '.webp': 'image/webp',
  }
  const extension = path.extname(imagePath).toLowerCase()
  const mimeType = mimeTypes[extension]
  if (!mimeType) throw new Error(`Unsupported image type: ${imagePath}`)

  return mimeType
}

// The temporary workspace keeps Slidev's runtime files out of `lectures/`. Polling the original
// Markdown path also survives editors that save by replacing the file instead of updating it.
function watchSourceDeck(source, destination) {
  let pendingCopy = Promise.resolve()
  const copyUpdatedSource = (current, previous) => {
    if (current.mtimeMs === previous.mtimeMs) return

    pendingCopy = pendingCopy
      .then(() => inlineMarkdownImages(source, destination))
      .catch((error) => console.error(`Failed to refresh ${destination}:`, error))
  }

  watchFile(source, { interval: 250 }, copyUpdatedSource)
  return () => unwatchFile(source, copyUpdatedSource)
}

// Slidev exports through Playwright. Prefer an explicit override, then browsers on PATH, then the
// standard installation paths used by Chrome and Chromium on desktop platforms.
function browserArgs() {
  const browser = resolveSystemBrowser()
  return browser ? ['--executable-path', browser] : []
}

function resolveSystemBrowser() {
  const candidates = [
    process.env.STUCO_SLIDEV_CHROME,
    ...browsersOnPath(),
    ...platformBrowserPaths(),
  ].filter(Boolean)

  for (const candidate of candidates) {
    try {
      accessSync(candidate, constants.X_OK)
      return candidate
    } catch {
      // Keep looking; falling through leaves Playwright to use its downloaded browser.
    }
  }

  return undefined
}

function browsersOnPath() {
  const candidates = []
  const directories = (process.env.PATH ?? '').split(path.delimiter).filter(Boolean)
  for (const name of [
    'google-chrome-stable',
    'google-chrome',
    'chromium',
    'chromium-browser',
  ]) {
    for (const directory of directories) {
      candidates.push(path.join(directory, name))
    }
  }

  return candidates
}

function platformBrowserPaths() {
  if (process.platform === 'darwin') {
    return [
      '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
      '/Applications/Chromium.app/Contents/MacOS/Chromium',
    ]
  }

  if (process.platform === 'win32') {
    const roots = [
      process.env.LOCALAPPDATA,
      process.env.PROGRAMFILES,
      process.env['PROGRAMFILES(X86)'],
    ].filter(Boolean)
    return roots.flatMap((root) => [
      path.join(root, 'Google', 'Chrome', 'Application', 'chrome.exe'),
      path.join(root, 'Chromium', 'Application', 'chrome.exe'),
    ])
  }

  return []
}

function runSlidev(args) {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, [slidevCli, ...args], {
      cwd: slidevRoot,
      env: {
        ...process.env,
        STUCO_SLIDEV_REPOSITORY_ROOT: repositoryRoot,
      },
      stdio: 'inherit',
    })

    const forwardSignal = (signal) => child.kill(signal)
    const onInterrupt = () => forwardSignal('SIGINT')
    const onTerminate = () => forwardSignal('SIGTERM')
    process.once('SIGINT', onInterrupt)
    process.once('SIGTERM', onTerminate)

    child.once('error', reject)
    child.once('exit', (code, signal) => {
      process.removeListener('SIGINT', onInterrupt)
      process.removeListener('SIGTERM', onTerminate)

      if (signal === 'SIGINT') resolve(130)
      else if (signal === 'SIGTERM') resolve(143)
      else resolve(code ?? 1)
    })
  })
}
