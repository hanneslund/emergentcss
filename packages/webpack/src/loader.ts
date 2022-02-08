export default function EmergentcssWebpackLoader(source: string) {
  let ecssComment = "";
  function replaceEcss(source: string) {
    const ecssIndex = source.indexOf("ecss`");
    if (ecssIndex === -1) {
      return source;
    }

    const before = `${source.slice(0, ecssIndex)}"`;
    let after = source.slice(ecssIndex + 5);

    const ecssEndIndex = after.indexOf("`");
    if (ecssEndIndex === -1) {
      throw new SyntaxError();
    }

    const ecss = transform(after.slice(0, ecssEndIndex));
    ecssComment += ` ${ecss}`;
    after = `"${after.slice(ecssEndIndex + 1)}`;

    return `${before}${ecss}${replaceEcss(after)}`;
  }

  let newSource = replaceEcss(source);
  if (ecssComment.length > 0) {
    newSource = `/*ecss:${ecssComment}*/\n${newSource}`;
  }

  return newSource;
}

function transform(ecss: string) {
  const arr = ecss
    .split("")
    .filter((c) => c !== "\n")
    .map((c) => c.toUpperCase());
  return arr.join("");
}
