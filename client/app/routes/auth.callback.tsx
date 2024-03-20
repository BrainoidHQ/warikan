import { type LoaderFunction } from "@vercel/remix";
import { authenticator } from "~/services/auth.server";

export const loader: LoaderFunction = ({ request }) => {
  return authenticator.authenticate("auth0", request, {
    successRedirect: "/",
    failureRedirect: "/auth/signin",
  });
};
