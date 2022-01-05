type Props = {
  name: string;
  colors: string[];
};

export default function ColorRow({ name, colors }: Props) {
  return (
    <div ecss="m-t=8">
      {name}
      <div ecss="m-t=2 grid (_ @lg):(grid-(c=(5 10) r=(2 1))) font-s=sm gap=2 font-s=xs">
        {colors.map((color, i) => (
          <div key={color}>
            <div
              ecss="h=10 w=full border-r=lg bg-c=[#f8fafc]"
              style={{ background: color }}
            />
            <div ecss="m-t=1 font-w=700 text-a=center">
              {Math.max(50, i * 100)}
            </div>
            <div ecss="text-c=neutral-400 text-a=center">{color}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
