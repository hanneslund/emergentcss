import { ReactNode } from "react";

type Props = {
  children: ReactNode;
};

export default function Container({ children }: Props) {
  return <div ecss="m-x=auto max-w=screen-lg p-x=6">{children}</div>;
}
