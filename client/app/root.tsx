import { NextUIProvider } from "@nextui-org/react";
import { type LinksFunction } from "@vercel/remix";
import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  useNavigate,
} from "@remix-run/react";
import { Analytics } from "@vercel/analytics/react";
import stylesheet from "~/tailwind.css?url";
import { Container } from "~/components/Container";

export const links: LinksFunction = () => [
  { rel: "stylesheet", href: stylesheet },
];

export function Layout({ children }: { children: React.ReactNode }) {
  const navigate = useNavigate();

  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        <NextUIProvider navigate={navigate}>{children}</NextUIProvider>
        <ScrollRestoration />
        <Analytics />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return (
    <Container>
      <Outlet />
    </Container>
  );
}
