import { ActionFunction, redirect } from "@remix-run/node";

import { GraphQLClient } from 'graphql-request';
import { parseWithZod } from '@conform-to/zod';

import { API_URL } from "~/services/constants.server";
import { authenticator } from '~/services/auth.server';
import { CreateGroupMutation } from "~/lib/query";
import { CreateGroupMutationSchema as schema } from "~/lib/form";

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
  const result = await client.request(CreateGroupMutation, { input: parsedFormData.value })
  if (!result.createGroup) {
    throw new Response("Not Found", { status: 404 });
  }

  return redirect(`/groups/${result.createGroup.id}`);
};
