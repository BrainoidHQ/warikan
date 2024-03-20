import { type SerializeFrom } from "@vercel/remix"
import { z } from 'zod';
import { GetPaymentDetailQuery } from '~/gql/graphql';

export const CreateUserMutationSchema = z.object({
  name: z.string(),
});

export const CreateGroupMutationSchema = z.object({
  title: z.string(),
});

export const UpdatePaymentMutationSchema = z.object({
  title: z.string(),
  creditors: z.array(
    z.object({
      user: z.string(),
      amount: z.number(),
    })
  ),
  debtors: z.array(
    z.object({
      user: z.string(),
      amount: z.number(),
    })
  ),
});

export const UpdatePaymentMutationDefaultValue = (value: SerializeFrom<NonNullable<GetPaymentDetailQuery["payment"]>>) => ({
  title: value.title,
  creditors: value.creditors.map((creditor) => ({
    user: creditor.user.id,
    amount: creditor.amount,
  })),
  debtors: value.debtors.map((debtor) => ({
    user: debtor.user.id,
    amount: debtor.amount,
  })),
});
