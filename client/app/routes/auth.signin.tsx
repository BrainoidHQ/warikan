import { type ActionFunction } from "@remix-run/node";
import { Form } from "@remix-run/react";

import { Button } from "@nextui-org/react";

import { authenticator } from "~/services/auth.server";
import { AppBar } from "~/components/AppBar";

export const action: ActionFunction = ({ request }) => {
  return authenticator.authenticate("auth0", request);
};

export default function SignIn() {
  return (
    <>
      <AppBar
        breadcrumbs={[
          { label: "ホーム", href: "/" },
          { label: "サインイン", href: "/auth/signin" },
        ]}
        buttons={[]}
      />

      <div className="flex flex-col gap-4">
        <Form method="post">
          <Button color="primary" type="submit">サインイン</Button>
        </Form>
      </div>
    </>
  )
}
