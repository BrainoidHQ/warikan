import { type LoaderFunctionArgs, json, redirect } from "@vercel/remix";
import { useLoaderData } from "@remix-run/react";

import { compareDesc, format } from "date-fns";
import { GraphQLClient } from "graphql-request";

import {
  Button,
  Card,
  CardBody,
  CardHeader,
  Divider,
  Listbox,
  ListboxItem,
  useDisclosure,
} from "@nextui-org/react";
import { GearIcon, PlusIcon } from "@radix-ui/react-icons";

import { API_URL } from "~/services/constants.server";
import { AppBar } from "~/components/AppBar";
import { authenticator } from "~/services/auth.server";
import { CreateGroupModal } from "~/components/CreateGroupModal";
import { GetUserDetailQuery } from "~/lib/query";
import { UnimplementedModal } from "~/components/UnimplementedModal";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const user = await authenticator.isAuthenticated(request, {
    failureRedirect: "/auth/signin",
  });

  const client = new GraphQLClient(API_URL, {
    fetch,
    headers: { authorization: `Bearer ${user.token}` },
  });
  const result = await client.request(GetUserDetailQuery, { id: user.id });
  if (!result.user) {
    return redirect(`/auth/signup`);
  }
  if (!result.groups) {
    throw new Response("Not Found", { status: 404 });
  }

  return json({ user: result.user, groups: result.groups });
};

export default function App() {
  const groups = useLoaderData<typeof loader>();

  const {
    isOpen: isSettingModalOpen,
    onOpen: onSettingModalOpen,
    onOpenChange: onSettingModalOpenChange,
  } = useDisclosure();
  const {
    isOpen: isCreateGroupModalOpen,
    onOpen: onCreateGroupModalOpen,
    onOpenChange: onCreateGroupModalOpenChange,
  } = useDisclosure();

  return (
    <>
      <AppBar
        breadcrumbs={[{ label: "ホーム", href: `/` }]}
        buttons={[
          { label: "設定", icon: <GearIcon />, onPress: onSettingModalOpen },
        ]}
      />

      <div className="flex flex-col gap-4">
        <Card shadow="sm">
          <CardHeader className="justify-between">
            <h3>グループ一覧</h3>
            <Button
              isIconOnly
              size="sm"
              variant="bordered"
              color="primary"
              onPress={onCreateGroupModalOpen}
            >
              <PlusIcon />
            </Button>
          </CardHeader>
          <Divider />
          <CardBody>
            <Listbox>
              {groups.groups
                .sort((a, b) =>
                  compareDesc(new Date(a.createdAt), new Date(b.createdAt)),
                )
                .map((group) => (
                  <ListboxItem
                    key={group.id}
                    href={`/groups/${group.id}`}
                    description={format(
                      new Date(group.createdAt),
                      "yyyy年MM月dd日",
                    )}
                  >
                    {group.title}
                  </ListboxItem>
                ))}
            </Listbox>
          </CardBody>
        </Card>
      </div>

      <UnimplementedModal
        isOpen={isSettingModalOpen}
        onOpenChange={onSettingModalOpenChange}
      />

      <CreateGroupModal
        isOpen={isCreateGroupModalOpen}
        onOpenChange={onCreateGroupModalOpenChange}
      />
    </>
  );
}
