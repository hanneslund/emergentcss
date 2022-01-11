import React, { ReactNode, useState } from "react";
import * as Accordion from "@radix-ui/react-accordion";
import styles from "./utils.module.css";

type ValuesFn = (
  properties: string[]
) => { value: ReactNode; css: ReactNode }[];
const auto: ValuesFn = (properties) => [
  { value: "auto", css: properties.map((prop) => `${prop}: auto;`).join("\n") },
];

const spacing: ValuesFn = (properties) => [
  { value: "0", css: properties.map((prop) => `${prop}: 0px;`).join("\n") },
  { value: "px", css: properties.map((prop) => `${prop}: 1px;`).join("\n") },
  {
    value: "{number}",
    css: properties.map((prop) => `${prop}: {number/4}rem;`).join("\n"),
  },
];

const percentage: ValuesFn = (properties) => [
  { value: "full", css: properties.map((prop) => `${prop}: 100%;`).join("\n") },
  {
    value: "{n1}/{n2}",
    css: properties.map((prop) => `${prop}: {n1/n2*100}%;`).join("\n"),
  },
];

type Utils = Array<{
  header: string;
  utils: Array<{
    name: string;
    properties: string[];
    valueFns?: ValuesFn[];
    values?: Array<{ value: string; css: string }>;
  }>;
}>;
const utils: Utils = [
  {
    header: "Positioning",
    utils: [
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
    ],
  },

  {
    header: "Spacing",
    utils: [
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
      {
        name: "p",
        properties: ["padding"],
        valueFns: [spacing],
      },
      {
        name: "p-t",
        properties: ["padding-top"],
        valueFns: [spacing],
      },
      {
        name: "p-r",
        properties: ["padding-right"],
        valueFns: [spacing],
      },
      {
        name: "p-b",
        properties: ["padding-bottom"],
        valueFns: [spacing],
      },
      {
        name: "p-l",
        properties: ["padding-left"],
        valueFns: [spacing],
      },
      {
        name: "p-x",
        properties: ["padding-left", "padding-right"],
        valueFns: [spacing],
      },
      {
        name: "p-y",
        properties: ["padding-top", "padding-bottom"],
        valueFns: [spacing],
      },
    ],
  },
  {
    header: "Sizing",
    utils: [
      {
        name: "w",
        properties: ["width"],
        values: [{ value: "screen", css: "height: 100vw;" }],
        valueFns: [spacing, percentage],
      },
      {
        name: "max-w",
        properties: ["max-width"],
        values: [
          { value: "0", css: "0rem" },
          { value: "none", css: "none" },
          { value: "xs", css: "20rem" },
          { value: "sm", css: "24rem" },
          { value: "md", css: "28rem" },
          { value: "lg", css: "32rem" },
          { value: "xl", css: "36rem" },
          { value: "2xl", css: "42rem" },
          { value: "3xl", css: "48rem" },
          { value: "4xl", css: "56rem" },
          { value: "5xl", css: "64rem" },
          { value: "6xl", css: "72rem" },
          { value: "7xl", css: "80rem" },
          { value: "full", css: "100%" },
          { value: "min", css: "min-content" },
          { value: "max", css: "max-content" },
          { value: "fit", css: "fit-content" },
          { value: "prose", css: "65ch" },
          { value: "screen-sm", css: "640px" },
          { value: "screen-md", css: "768px" },
          { value: "screen-lg", css: "1024px" },
          { value: "screen-xl", css: "1280px" },
          { value: "screen-2xl", css: "1536px" },
        ],
      },
      {
        name: "h",
        properties: ["height"],
        values: [{ value: "screen", css: "height: 100vh;" }],
        valueFns: [spacing, percentage],
      },
    ],
  },
];

export default function Utils() {
  const [filter, setFilter] = useState("");
  const filteredUtils = utils
    .map(({ header, utils }) => ({
      header,
      utils: utils.filter(
        ({ name, properties }) =>
          !name ||
          name.toUpperCase().includes(filter.toUpperCase()) ||
          properties.some((prop) =>
            prop.toUpperCase().includes(filter.toUpperCase())
          )
      ),
    }))
    .filter(({ utils }) => utils.length > 0);

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

      {filteredUtils.length === 0 ? (
        <></>
      ) : (
        <div ecss="font-s=sm m-t=6 border-(w=1 c=neutral-800 r=md) overflow=hidden">
          <Accordion.Root type="multiple" className={styles.accordion}>
            {filteredUtils.map(({ header, utils }) => (
              <React.Fragment key={header}>
                <h2
                  key={header}
                  ecss="p=5 border-(tw=1 first:tw=0 c=neutral-800) bg-c=[#0f0f0f]"
                >
                  {header}
                </h2>
                {utils.map(
                  ({ name, properties, values = [], valueFns = [] }) => (
                    <Accordion.Item key={name} value={`${name}`}>
                      <Accordion.Header>
                        <Accordion.Trigger asChild>
                          <button ecss="w=full border-(tw=1 c=neutral-800) hover:bg-c=neutral-900 p=4 w=full text-a=left flex justify-c=between">
                            <div>
                              <span ecss="font-w=bold text-c=blue-200">
                                {name}
                              </span>
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
                              {values.map(({ value, css }) => (
                                <ValueRow
                                  key={name + value}
                                  name={name}
                                  value={value}
                                  css={css}
                                />
                              ))}
                              {valueFns.flatMap((fn) =>
                                fn(properties).map(({ value, css }) => (
                                  <ValueRow
                                    key={name + value}
                                    name={name}
                                    value={value}
                                    css={css}
                                  />
                                ))
                              )}
                            </tbody>
                          </table>
                        </div>
                      </Accordion.Content>
                    </Accordion.Item>
                  )
                )}
              </React.Fragment>
            ))}
          </Accordion.Root>
        </div>
      )}
    </div>
  );
}

type ValueRowProps = {
  name: ReactNode;
  value: ReactNode;
  css: ReactNode;
};
const ValueRow = ({ name, value, css }: ValueRowProps) => (
  <tr ecss="odd:bg-c=[#0f0f0f]">
    <td ecss="p=2 text-(c=neutral-400 first:c=neutral-200)">
      {name}={value}
    </td>
    <td ecss="p=2 text-(c=neutral-400 first:c=neutral-200) [white-space: pre]">
      {css}
    </td>
  </tr>
);
