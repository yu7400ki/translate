import init from "marian";
import { useEffect, useState } from "react";

type Props = {
  children: React.ReactNode;
};

export default function InitWasm({ children }: Props) {
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    init().then(() => setLoaded(true));
  }, []);

  return loaded ? <>{children}</> : null;
}
