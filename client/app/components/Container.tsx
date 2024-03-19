import { ReactNode } from "react";

export function Container(props: { children: ReactNode }) {
  return (
    <div className="max-w-screen-sm mx-auto">
      {props.children}
    </div>
  )
}
