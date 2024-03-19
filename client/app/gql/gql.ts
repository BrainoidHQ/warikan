/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query GetUserDetail($id: ID!) {\n    user(id: $id) {\n      id\n      createdAt\n      updatedAt\n      name\n    }\n    groups(id: $id) {\n      id\n      createdAt\n      title\n    }\n  }\n": types.GetUserDetailDocument,
    "\n  mutation CreateUser($input: CreateUserInput!) {\n    createUser(input: $input) {\n      id\n    }\n  }\n": types.CreateUserDocument,
    "\n  query GetGroupDetail($id: ID!) {\n    group(id: $id) {\n      id\n      createdAt\n      title\n      participants {\n        id\n        name\n      }\n      payments {\n        id\n        createdAt\n        title\n        creditors {\n          user {\n            id\n            name\n          }\n          amount\n        }\n      }\n    }\n  }\n": types.GetGroupDetailDocument,
    "\n  mutation CreateGroup($input: CreateGroupInput!) {\n    createGroup(input: $input) {\n      id\n    }\n  }\n": types.CreateGroupDocument,
    "\n  query GetPaymentDetail($id: ID!) {\n    payment(id: $id) {\n      id\n      createdAt\n      title\n      creditors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n      debtors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n    }\n  }\n": types.GetPaymentDetailDocument,
    "\n  mutation CreatePayment($input: CreatePaymentInput!) {\n    createPayment(input: $input) {\n      id\n    }\n  }\n": types.CreatePaymentDocument,
    "\n  mutation UpdatePayment($input: UpdatePaymentInput!) {\n    updatePayment(input: $input) {\n      id\n    }\n  }\n": types.UpdatePaymentDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetUserDetail($id: ID!) {\n    user(id: $id) {\n      id\n      createdAt\n      updatedAt\n      name\n    }\n    groups(id: $id) {\n      id\n      createdAt\n      title\n    }\n  }\n"): (typeof documents)["\n  query GetUserDetail($id: ID!) {\n    user(id: $id) {\n      id\n      createdAt\n      updatedAt\n      name\n    }\n    groups(id: $id) {\n      id\n      createdAt\n      title\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateUser($input: CreateUserInput!) {\n    createUser(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateUser($input: CreateUserInput!) {\n    createUser(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetGroupDetail($id: ID!) {\n    group(id: $id) {\n      id\n      createdAt\n      title\n      participants {\n        id\n        name\n      }\n      payments {\n        id\n        createdAt\n        title\n        creditors {\n          user {\n            id\n            name\n          }\n          amount\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetGroupDetail($id: ID!) {\n    group(id: $id) {\n      id\n      createdAt\n      title\n      participants {\n        id\n        name\n      }\n      payments {\n        id\n        createdAt\n        title\n        creditors {\n          user {\n            id\n            name\n          }\n          amount\n        }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreateGroup($input: CreateGroupInput!) {\n    createGroup(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreateGroup($input: CreateGroupInput!) {\n    createGroup(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query GetPaymentDetail($id: ID!) {\n    payment(id: $id) {\n      id\n      createdAt\n      title\n      creditors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n      debtors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetPaymentDetail($id: ID!) {\n    payment(id: $id) {\n      id\n      createdAt\n      title\n      creditors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n      debtors {\n        user {\n          id\n          name\n        }\n        amount\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation CreatePayment($input: CreatePaymentInput!) {\n    createPayment(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation CreatePayment($input: CreatePaymentInput!) {\n    createPayment(input: $input) {\n      id\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation UpdatePayment($input: UpdatePaymentInput!) {\n    updatePayment(input: $input) {\n      id\n    }\n  }\n"): (typeof documents)["\n  mutation UpdatePayment($input: UpdatePaymentInput!) {\n    updatePayment(input: $input) {\n      id\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;