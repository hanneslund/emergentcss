import { MDXProvider } from "@mdx-js/react";
import Layout from "../components/Layout";
import "../styles/globals.css";

const components = {
  // img: ResponsiveImage,
  h1: ({ children }) => <h1 ecss="font-(s=4xl w=semibold)">{children}</h1>,
  h2: ({ children }) => (
    <h1 ecss="m-(t=12 b=6) font-(s=2xl w=semibold)">{children}</h1>
  ),
  h3: ({ children }) => (
    <h1 ecss="m-(t=12 b=6) font-(s=xl w=semibold)">{children}</h1>
  ),
  h4: ({ children }) => (
    <h1 ecss="m-(t=12 b=6) font-(s=lg w=semibold)">{children}</h1>
  ),
  p: ({ children }) => <p ecss="m-y=4 text-c=neutral-300">{children}</p>,
  inlineCode: ({ children }) => (
    <code ecss="bg-c=gray-900 text-c=blue-100 p-(y=0.5 x=1) border-r=md font-s=sm">
      {children}
    </code>
  ),
  a: ({ children, href }) => (
    <a
      ecss="text-(c=blue-400 hover:c=blue-300 underline) [text-underline-offset: 2px]"
      href={href}
      target="_blank"
      rel="noreferrer"
    >
      {children}
    </a>
  ),
  ul: ({ children }) => (
    <ul ecss="list-disc m-x=6 text-c=neutral-300">{children}</ul>
  ),
  li: ({ children }) => <li ecss="m-b=2">{children}</li>,
  pre: ({ children }) => (
    <pre ecss="bg-c=gray-900 text-c=blue-100 p=4 m-y=6 border-r=md overflow-x=auto">
      {children}
    </pre>
  ),
  table: ({ children }) => (
    <table ecss="w=full text-a=left font-s=sm">{children}</table>
  ),
  tr: ({ children }) => <tr ecss="border-(bw=1 c=neutral-800)">{children}</tr>,
  td: ({ children }) => (
    <td ecss="p-y=2 text-(c=neutral-400 first:c=blue-200)">{children}</td>
  ),
  th: ({ children }) => (
    <td ecss="p-y=2 font-w=bold border-(bw=1 c=neutral-600)">{children}</td>
  ),
};

function MyApp({ Component, pageProps }) {
  return (
    <MDXProvider components={components}>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </MDXProvider>
  );
}

export default MyApp;
