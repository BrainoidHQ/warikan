import { type ActionFunction, redirect } from "@vercel/remix";

import { GraphQLClient } from 'graphql-request';
import invariant from "tiny-invariant";

import { API_URL } from "~/services/constants.server";
import { authenticator } from '~/services/auth.server';
import { CreatePaymentMutation } from "~/lib/query";

export const action: ActionFunction = async ({ request, params }) => {
  invariant(params.groupId, "Missing groupId param");

  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } })
  const result = await client.request(CreatePaymentMutation, { input: { group: params.groupId, title: "新規支払い" } })
  if (!result.createPayment) {
    throw new Response("Not Found", { status: 404 });
  }

  return redirect(`/groups/${params.groupId}/payments/${result.createPayment.id}`);
};
