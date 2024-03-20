import { type LoaderFunctionArgs, json } from "@vercel/remix";

import { GraphQLClient } from 'graphql-request';
import invariant from "tiny-invariant";

import { API_URL } from "~/services/constants.server";
import { authenticator } from '~/services/auth.server';
import { GetGroupDetailQuery } from "~/lib/query";

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
  invariant(params.groupId, "Missing groupId param");

  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } })
  const result = await client.request(GetGroupDetailQuery, { id: params.groupId })
  if (!result.group) {
    throw new Response("Not Found", { status: 404 });
  }
  return json({ group: result.group });
};
