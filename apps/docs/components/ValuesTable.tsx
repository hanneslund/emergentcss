import { ReactNode } from "react";
type Values = { name: ReactNode; value: ReactNode }[];

type Props = {
  values: Values;
};
export default function ValuesTable({ values }: Props) {
  return (
    <table ecss="w=full text-a=left font-s=sm">
      <thead>
        <tr ecss="border-(bw=1 c=neutral-800)">
          <th ecss="p-y=2 border-(bw=1 c=neutral-600)">Value</th>
          <th ecss="p-y=2 border-(bw=1 c=neutral-600)">CSS</th>
        </tr>
      </thead>
      <tbody>
        {values.map(({ name, value }, i) => (
          <tr ecss="border-(bw=1 c=neutral-800)" key={i}>
            <td ecss="p-y=2 text-(c=neutral-400 first:c=blue-200)">{name}</td>
            <td ecss="p-y=2 text-(c=neutral-400 first:c=blue-200)">{value}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export const spacing: (prop: string) => Values = (prop) => [
  { name: "0", value: `${prop}: 0px;` },
  { name: "px", value: `${prop}: 1px;` },
  { name: "{number}", value: `${prop}: {number/4}rem;` },
];

export const percentage: (prop: string) => Values = (prop) => [
  { name: "full", value: `${prop}: 100%;` },
  { name: "{n1}/{n2}", value: `${prop}: {n1/n2*100}%;` },
];
