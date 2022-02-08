import path from "path";
import webpack from "webpack";
import EmergentcssWebpackPlugin from "../src/plugin";

const run = (fixture) => {
  const compiler = webpack({
    mode: "production",
    context: __dirname,
    entry: `./fixtures/${fixture}/index.js`,
    output: {
      path: path.resolve(__dirname),
      filename: "bundle.js",
    },
    module: {
      rules: [
        {
          test: /\.js$/,
          use: path.resolve(__dirname, "../src/loader.ts"),
        },
      ],
    },
    plugins: [new EmergentcssWebpackPlugin()],
  });

  // compiler.outputFileSystem = createFsFromVolume(new Volume());
  // compiler.outputFileSystem.join = path.join.bind(path);

  return new Promise((resolve, reject) => {
    compiler.run((err, stats) => {
      if (err) reject(err);
      if (stats.hasErrors()) reject(stats.toJson().errors);

      resolve(stats);
    });
  });
};

test("basic", async () => {
  await run("basic");
});
