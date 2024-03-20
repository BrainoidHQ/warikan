import { useEffect, useRef } from 'react';
import { type LoaderFunctionArgs, type ActionFunction, json, redirect } from "@remix-run/node";
import { Form, useActionData, useLoaderData, useRouteLoaderData } from "@remix-run/react";
import { useDebounceSubmit } from "remix-utils/use-debounce-submit";

import { SubmissionResult, getFormProps, useForm } from "@conform-to/react";
import { getZodConstraint, parseWithZod } from '@conform-to/zod';
import { GraphQLClient } from 'graphql-request';
import invariant from "tiny-invariant";

import {
  Button,
  Card,
  CardBody,
  CardHeader,
  Divider,
  Input,
  useDisclosure,
} from "@nextui-org/react";
import { Share2Icon, GearIcon, PlusIcon } from '@radix-ui/react-icons'

import { AmountInput } from "~/components/AmountInput";
import { API_URL } from "~/services/constants.server";
import { AppBar } from "~/components/AppBar";
import { authenticator } from '~/services/auth.server';
import { GetPaymentDetailQuery , UpdatePaymentMutation } from "~/lib/query";
import { loader as groupLoader } from "~/routes/groups.$groupId";
import { UnimplementedModal } from "~/components/UnimplementedModal";
import { UpdatePaymentMutationDefaultValue, UpdatePaymentMutationSchema } from "~/lib/form";

export const loader = async ({ request, params }: LoaderFunctionArgs) => {
  invariant(params.paymentId, "Missing paymentId param");

  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } })
  const result = await client.request(GetPaymentDetailQuery, { id: params.paymentId })
  if (!result.payment) {
    throw new Response("Not Found", { status: 404 });
  }

  return json({ payment: result.payment });
};

export const action: ActionFunction = async ({ request, params }) => {
  invariant(params.paymentId, "Missing paymentId param");

  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  const formData = await request.formData();
  const parsedFormData = parseWithZod(formData, { schema: UpdatePaymentMutationSchema });
  if (parsedFormData.status !== 'success') {
    return parsedFormData.reply();
  }

  const client = new GraphQLClient(API_URL, { fetch, headers: { authorization: `Bearer ${user.token}` } })
  const result = await client.request(UpdatePaymentMutation, { input: { id: params.paymentId, ...parsedFormData.value } })
  if (!result.updatePayment) {
    throw new Response("Not Found", { status: 404 });
  }

  return redirect(`/groups/${params.groupId}/payments/${result.updatePayment.id}`);
};

export default function PaymentDetail() {
  const groupLoaderData = useRouteLoaderData<typeof groupLoader>("routes/groups.$groupId");
  invariant(groupLoaderData?.group, "Missing data");
  const group = groupLoaderData.group;

  const paymentLoaderData = useLoaderData<typeof loader>();
  invariant(paymentLoaderData?.payment, "Missing data");
  const payment = paymentLoaderData.payment;

  const formRef = useRef<HTMLFormElement>(null);
  const submit = useDebounceSubmit();
  // TODO: improve type annotation
  const lastResult: SubmissionResult<string[]> | null | undefined = useActionData<typeof action>();
  const [form, fields] = useForm({
    lastResult,
    constraint: getZodConstraint(UpdatePaymentMutationSchema),
    defaultValue: UpdatePaymentMutationDefaultValue(payment),
  });

  // TODO: remove useEffect
  useEffect(() => {
    if (formRef.current && lastResult?.intent?.type === "remove") {
      submit(formRef.current, {
        navigate: false,
        fetcherKey: form.id,
        debounceTimeout: 0,
      })
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [formRef, lastResult, form.initialValue])

  const {isOpen, onOpen, onOpenChange} = useDisclosure();

  return (
    <>
      <AppBar
        breadcrumbs={[
          { label: "ホーム", href: `/` },
          { label: group.title, href: `/groups/${group.id}` },
          { label: payment.title, href: `/groups/${group.id}/payments/${payment.id}` },
        ]}
        buttons={[
          { label: "共有", icon: <Share2Icon />, onPress: onOpen },
          { label: "設定", icon: <GearIcon />, onPress: onOpen },
        ]}
      />

      <Form method="post" {...getFormProps(form)} className="flex flex-col gap-4" ref={formRef}>
        <Input
          name={fields.title.name}
          aria-invalid={fields.title.errors ? true : undefined}
					aria-describedby={fields.title.errors ? fields.title.errorId : undefined}
          onChange={(event) => {
            submit(event.target.form, {
              navigate: false,
              fetcherKey: fields.title.id,
              debounceTimeout: 1000,
            });
          }}
          onBlur={(event) => {
            submit((event.target as HTMLInputElement).form, {
              navigate: false,
              fetcherKey: fields.title.id,
              debounceTimeout: 0,
            });
          }}
          defaultValue={fields.title.initialValue}
          label="タイトル"
        />

        <Card shadow="sm">
          <CardHeader className="justify-between">
            <h3>支払った人</h3>
            <Button
              {...form.insert.getButtonProps({
                name: fields.creditors.name,
              })}
              type="submit"
              isIconOnly
              size="sm"
              variant="bordered"
              color="primary"
            >
              <PlusIcon />
            </Button>
          </CardHeader>
          <Divider />
          <CardBody>
            <ul className="flex flex-col gap-2">
              {fields.creditors.getFieldList().map((creditor, index) => (
                <AmountInput
                  key={creditor.key}
                  amount={creditor}
                  participants={group.participants}
                  removeButton={form.remove.getButtonProps({
                    name: fields.creditors.name,
                    index: index,
                  })}
                />
              ))}
            </ul>
          </CardBody>
        </Card>

        <Card shadow="sm">
          <CardHeader className="justify-between">
            <h3>支払うべき人</h3>
            <Button
              {...form.insert.getButtonProps({
                name: fields.debtors.name,
              })}
              type="submit"
              isIconOnly
              size="sm"
              variant="bordered"
              color="primary"
            >
              <PlusIcon />
            </Button>
          </CardHeader>
          <Divider />
          <CardBody>
            <ul className="flex flex-col gap-2">
              {fields.debtors.getFieldList().map((debtor, index) => (
                <AmountInput
                  key={debtor.key}
                  amount={debtor}
                  participants={group.participants}
                  removeButton={form.remove.getButtonProps({
                    name: fields.debtors.name,
                    index: index,
                  })}
                />
              ))}
            </ul>
          </CardBody>
        </Card>
      </Form>

      <UnimplementedModal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
      />
    </>
  );
}
