import webpack, { sources } from "webpack";

const PLUGIN_NAME = "EmergentcssWebpackPlugin";

class EmergentcssWebpackPlugin {
  utils = new Set<string>();

  apply(compiler: webpack.Compiler) {
    compiler.hooks.normalModuleFactory.tap(
      PLUGIN_NAME,
      (normalModuleFactory) => {
        const handler = (parser: webpack.javascript.JavascriptParser) => {
          parser.hooks.program.tap(PLUGIN_NAME, (_, comments) => {
            const ecss = comments.find(({ value }) =>
              value.startsWith("ecss: ")
            );
            if (ecss) {
              ecss.value
                .slice(6)
                .split(" ")
                .forEach((util) => {
                  console.log({ util });
                  this.utils.add(util);
                });

              console.log(this.utils);
            }
          });
        };

        normalModuleFactory.hooks.parser
          .for("javascript/auto")
          .tap(PLUGIN_NAME, handler);
        normalModuleFactory.hooks.parser
          .for("javascript/dynamic")
          .tap(PLUGIN_NAME, handler);
        normalModuleFactory.hooks.parser
          .for("javascript/esm")
          .tap(PLUGIN_NAME, handler);
      }
    );

    compiler.hooks.thisCompilation.tap(PLUGIN_NAME, (compilation) => {
      compilation.hooks.processAssets.tapPromise(
        {
          name: PLUGIN_NAME,
          stage: webpack.Compilation.PROCESS_ASSETS_STAGE_ADDITIONS,
        },
        async () => {
          compilation.emitAsset(
            "test.css",
            new sources.RawSource([...this.utils].join("\n"))
          );
        }
      );
    });
  }
}

export default EmergentcssWebpackPlugin;
