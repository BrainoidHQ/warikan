import { type LoaderFunctionArgs, json } from "@vercel/remix";
import { Form, useLoaderData, useRouteLoaderData } from "@remix-run/react";

import { compareDesc, format } from "date-fns";
import invariant from "tiny-invariant";

import {
  Button,
  Card,
  CardBody,
  CardHeader,
  Divider,
  Input,
  Listbox,
  ListboxItem,
  useDisclosure,
} from "@nextui-org/react";
import { Share2Icon, GearIcon, PlusIcon } from '@radix-ui/react-icons'

import { AppBar } from "~/components/AppBar";
import { loader as groupLoader } from "~/routes/groups.$groupId";
import { UnimplementedModal } from "~/components/UnimplementedModal";
import { authenticator } from '~/services/auth.server';

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: '/auth/signin',
  });

  return json({ user });
};

export default function GroupDetail() {
  const groupLoaderData = useRouteLoaderData<typeof groupLoader>("routes/groups.$groupId");
  invariant(groupLoaderData?.group, "Missing data");
  const group = groupLoaderData.group;

  const { user } = useLoaderData<typeof loader>();

  const {isOpen, onOpen, onOpenChange} = useDisclosure();

  return (
    <>
      <AppBar
        breadcrumbs={[
          { label: "ホーム", href: `/` },
          { label: group.title, href: `/groups/${group.id}` },
        ]}
        buttons={[
          { label: "共有", icon: <Share2Icon />, onPress: onOpen },
          { label: "設定", icon: <GearIcon />, onPress: onOpen },
        ]}
      />

      <div className="flex flex-col gap-4">
        <Input
          isReadOnly
          label="タイトル"
          defaultValue={group.title}
        />

        <Card shadow="sm">
          <CardHeader>
            <h3>やること</h3>
          </CardHeader>
          <Divider />
          <CardBody>
            <Listbox>
              {
                group
                  .warikan
                  .filter((warikan) => warikan.from.id === user.id || warikan.to.id === user.id)
                  .map((warikan) => warikan.from.id === user.id ? `${warikan.to.name}に${warikan.amount}円おくる` : `${warikan.from.name}から${warikan.amount}円もらう`)
                  .map((message) => (
                    <ListboxItem key={message}>
                      {message}
                    </ListboxItem>
                  ))
              }
            </Listbox>
          </CardBody>
        </Card>

        <Form method="post" action={`/groups/${group.id}/payments/create`}>
          <Card shadow="sm">
            <CardHeader className="justify-between">
              <h3>決済一覧</h3>
              <Button isIconOnly size="sm" type="submit" variant="bordered" color="primary">
                <PlusIcon />
              </Button>
            </CardHeader>
            <Divider />
            <CardBody>
              <Listbox>
                {
                  group
                    .payments
                    .sort((a, b) => compareDesc(new Date(a.createdAt), new Date(b.createdAt)))
                    .map((payment) => (
                      <ListboxItem
                        key={payment.id}
                        href={`/groups/${group.id}/payments/${payment.id}`}
                        description={format(new Date(payment.createdAt), "yyyy年MM月dd日")}
                      >
                        {payment.title}
                      </ListboxItem>
                    ))
                }
              </Listbox>
            </CardBody>
          </Card>
        </Form>
      </div>

      <UnimplementedModal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
      />
    </>
  );
}
