import { spawn } from 'node:child_process'
import { rmSync } from 'node:fs'
import { copyFile, mkdir, rm, symlink } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import path from 'node:path'

const slidevRoot = fileURLToPath(new URL('../', import.meta.url))
const repositoryRoot = path.resolve(slidevRoot, '..')
const sourceDeck = path.join(
  repositoryRoot,
  'lectures',
  '01_introduction',
  'introduction.md',
)
const sourceImages = path.join(repositoryRoot, 'lectures', 'images')
const runtimeRoot = path.join(slidevRoot, 'runtime')
const outputRoot = path.join(slidevRoot, 'dist')
const siteOutputRoot = path.join(outputRoot, 'site')
const slidevCli = path.join(
  slidevRoot,
  'node_modules',
  '@slidev',
  'cli',
  'bin',
  'slidev.mjs',
)

const task = process.argv[2]
const forwardedArgs = process.argv.slice(3)

if (!task) {
  console.error('Usage: node scripts/run.mjs <dev|build|export:light|export:dark>')
  process.exitCode = 1
} else {
  await runTask(task, forwardedArgs)
}

async function runTask(taskName, extraArgs) {
  const workspaceRoot = path.join(slidevRoot, '.slidev-work')
  const lectureRoot = path.join(workspaceRoot, 'lectures', '01_introduction')
  const setupRoot = path.join(lectureRoot, 'setup')
  const workspaceImages = path.join(workspaceRoot, 'lectures', 'images')
  const workspaceDeck = path.join(lectureRoot, 'introduction.md')
  const cleanupWorkspace = () =>
    rmSync(workspaceRoot, { recursive: true, force: true })

  await rm(workspaceRoot, { recursive: true, force: true })
  process.once('exit', cleanupWorkspace)
  try {
    await mkdir(lectureRoot, { recursive: true })
    await mkdir(setupRoot, { recursive: true })
    await symlink(sourceDeck, workspaceDeck, 'file')
    await symlink(
      sourceImages,
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

    await mkdir(outputRoot, { recursive: true })

    const taskArgs = {
      dev: [workspaceDeck],
      build: [
        'build',
        workspaceDeck,
        '--base',
        '/slidev/week01/',
        '--out',
        siteOutputRoot,
        '--without-notes',
      ],
      'export:light': [
        'export',
        workspaceDeck,
        '--output',
        path.join(outputRoot, 'introduction-light.pdf'),
        '--with-toc',
        '--timeout',
        '60000',
      ],
      'export:dark': [
        'export',
        workspaceDeck,
        '--output',
        path.join(outputRoot, 'introduction-dark.pdf'),
        '--dark',
        '--with-toc',
        '--timeout',
        '60000',
      ],
    }[taskName]

    if (!taskArgs) {
      throw new Error(`Unknown Slidev task: ${taskName}`)
    }

    const exitCode = await runSlidev([...taskArgs, ...extraArgs])
    if (exitCode !== 0) process.exitCode = exitCode
  } finally {
    process.removeListener('exit', cleanupWorkspace)
    await rm(workspaceRoot, { recursive: true, force: true })
  }
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
