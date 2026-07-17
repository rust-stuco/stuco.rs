export default {
  slidev: {
    markdown: {
      markdownSetup(markdownIt) {
        const renderBulletListOpen =
          markdownIt.renderer.rules.bullet_list_open ||
          ((tokens, index, options, _environment, renderer) =>
            renderer.renderToken(tokens, index, options))
        const renderBulletListClose =
          markdownIt.renderer.rules.bullet_list_close ||
          ((tokens, index, options, _environment, renderer) =>
            renderer.renderToken(tokens, index, options))
        const isAnimatedList = (token) =>
          token.markup === '*' && token.level === 0

        markdownIt.renderer.rules.bullet_list_open = (
          tokens,
          index,
          options,
          environment,
          renderer,
        ) => {
          const list = renderBulletListOpen(
            tokens,
            index,
            options,
            environment,
            renderer,
          )

          return isAnimatedList(tokens[index])
            ? `<v-clicks depth="2">\n${list}`
            : list
        }

        markdownIt.renderer.rules.bullet_list_close = (
          tokens,
          index,
          options,
          environment,
          renderer,
        ) => {
          const list = renderBulletListClose(
            tokens,
            index,
            options,
            environment,
            renderer,
          )

          return isAnimatedList(tokens[index])
            ? `${list}</v-clicks>\n`
            : list
        }
      },
    },
  },
  server: {
    fs: {
      allow: [process.env.STUCO_SLIDEV_REPOSITORY_ROOT],
    },
  },
  build: {
    emptyOutDir: true,
  },
}
