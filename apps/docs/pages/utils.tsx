import { ReactNode, useState } from "react";
import * as Accordion from "@radix-ui/react-accordion";

type Values = { value: ReactNode; css: ReactNode }[];
const auto: (properties: string[]) => Values = (properties) => [
  { value: "auto", css: properties.map((prop) => `${prop}: auto;`).join("\n") },
];

const spacing: (properties: string[]) => Values = (properties) => [
  { value: "0", css: properties.map((prop) => `${prop}: 0px;`).join("\n") },
  { value: "px", css: properties.map((prop) => `${prop}: 1px;`).join("\n") },
  {
    value: "{number}",
    css: properties.map((prop) => `${prop}: {number/4}rem;`).join("\n"),
  },
];

const percentage: (properties: string[]) => Values = (properties) => [
  { value: "full", css: properties.map((prop) => `${prop}: 100%;`).join("\n") },
  {
    value: "{n1}/{n2}",
    css: properties.map((prop) => `${prop}: {n1/n2*100}%;`).join("\n"),
  },
];

const utils = [
  {
    name: "t",
    properties: ["top"],
    valueFns: [auto, spacing, percentage],
  },
  {
    name: "r",
    properties: ["right"],
    valueFns: [auto, spacing, percentage],
  },
  {
    name: "b",
    properties: ["bottom"],
    valueFns: [auto, spacing, percentage],
  },
  {
    name: "l",
    properties: ["left"],
    valueFns: [auto, spacing, percentage],
  },
  {
    name: "m",
    properties: ["margin"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-t",
    properties: ["margin-top"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-r",
    properties: ["margin-right"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-b",
    properties: ["margin-bottom"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-l",
    properties: ["margin-left"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-x",
    properties: ["margin-left", "margin-right"],
    valueFns: [auto, spacing],
  },
  {
    name: "m-y",
    properties: ["margin-top", "margin-bottom"],
    valueFns: [auto, spacing],
  },
];

export default function Utils() {
  const [filter, setFilter] = useState("");
  const filteredUtils = utils.filter(
    ({ name, properties }) =>
      name.includes(filter) || properties.some((prop) => prop.includes(filter))
  );
  return (
    <div>
      <h1 ecss="font-(s=4xl w=semibold)">Utils</h1>
      <input
        type="text"
        ecss="m-t=10 block bg-c=black p-(x=4 y=2) border-(r=md w=1 c=neutral-800)"
        placeholder="Filter"
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
      />

      <div ecss="font-s=sm m-t=6 border-(w=1 c=neutral-800 r=md)">
        <Accordion.Root type="multiple">
          {filteredUtils.map(({ name, properties, valueFns }) => (
            <Accordion.Item key={name} value={`${name}`}>
              <Accordion.Header>
                <Accordion.Trigger asChild>
                  <button ecss="w=full border-(tw=1 c=neutral-800) hover:bg-c=neutral-900 p=4 w=full text-a=left flex justify-c=between">
                    <div>
                      <span ecss="font-w=bold text-c=blue-200">{name}</span>
                      <span ecss="m-l=2 font-s=xs text-c=neutral-400">
                        {properties.join(", ")}
                      </span>
                    </div>
                    <svg
                      aria-hidden
                      viewBox="0 0 24 24"
                      width="24"
                      height="24"
                      stroke="currentColor"
                      strokeWidth="1"
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      fill="none"
                      shapeRendering="geometricPrecision"
                    >
                      <path d="M6 9l6 6 6-6"></path>
                    </svg>
                  </button>
                </Accordion.Trigger>
              </Accordion.Header>
              <Accordion.Content>
                <div ecss="p-(x=10 t=6 b=10) border-(tw=1 c=[#111])">
                  <table ecss="w=full text-a=left font-s=sm">
                    <thead>
                      <tr>
                        <th ecss="p=2">Value</th>
                        <th ecss="p=2">CSS</th>
                      </tr>
                    </thead>
                    <tbody>
                      {valueFns.flatMap((fn) =>
                        fn(properties).map(({ value, css }) => (
                          <tr key={name + value} ecss="odd:bg-c=[#0a0a0a]">
                            <td ecss="p=2 text-(c=neutral-400 first:c=neutral-200)">
                              {name}={value}
                            </td>
                            <td ecss="p=2 text-(c=neutral-400 first:c=neutral-200) [white-space: pre]">
                              {css}
                            </td>
                          </tr>
                        ))
                      )}
                    </tbody>
                  </table>
                </div>
              </Accordion.Content>
            </Accordion.Item>
          ))}
        </Accordion.Root>
      </div>
    </div>
  );
}
