import { type ActionFunction, type LoaderFunction, redirect } from "@remix-run/node";
import { Form, useActionData } from "@remix-run/react";

import { getFormProps, getInputProps, useForm } from "@conform-to/react";
import { GraphQLClient } from "graphql-request";
import { getZodConstraint, parseWithZod } from "@conform-to/zod";

import { Button, Card, CardBody, CardFooter, CardHeader, Divider, Input } from "@nextui-org/react";

import { API_URL } from "~/services/constants.server";
import { AppBar } from "~/components/AppBar";
import { authenticator } from "~/services/auth.server";
import { CreateUserMutation, GetUserDetailQuery } from "~/lib/query";
import { CreateUserMutationSchema as schema } from "~/lib/form";

export const loader: LoaderFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } });
  const result = await client.request(GetUserDetailQuery, { id: user.id })
  if (result.user) {
    return redirect(`/`);
  }

  return null;
};

export const action: ActionFunction = async ({ request }) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const formData = await request.formData();
  const parsedFormData = parseWithZod(formData, { schema });
  if (parsedFormData.status !== 'success') {
    return parsedFormData.reply();
  }

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } });
  const result = await client.request(CreateUserMutation, { input: parsedFormData.value })
  if (!result.createUser) {
    throw new Response("Not Found", { status: 404 });
  }

  return redirect(`/`);
};

export default function SignUp() {
  const lastResult = useActionData<typeof action>();
  const [form, fields] = useForm({
    lastResult,
    constraint: getZodConstraint(schema),
  })

  return (
    <>
      <AppBar
        breadcrumbs={[
          { label: "ホーム", href: `/` },
          { label: "アカウント作成", href: `/auth/signup` },
        ]}
        buttons={[]}
      />

      <div className="flex flex-col gap-4">
        <Form method="post" {...getFormProps(form)}>
          <Card shadow="sm">
            <CardHeader>
              <h3>アカウント作成</h3>
            </CardHeader>
            <Divider />
            <CardBody>
              <Input {...getInputProps(fields.name, { type: "text" })} label="名前" />
            </CardBody>
            <Divider />
            <CardFooter>
              <Button color="primary" type="submit">
                作成
              </Button>
            </CardFooter>
          </Card>
        </Form>
      </div>
    </>
  )
}
