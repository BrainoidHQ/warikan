import { Form, useActionData } from "@remix-run/react";

import { getZodConstraint } from "@conform-to/zod";
import { getFormProps, useForm } from "@conform-to/react";

import {
  Button,
  Input,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
} from "@nextui-org/react";

import { action } from "~/routes/groups.create";
import { CreateGroupMutationSchema } from "~/lib/form";

export interface CreateGroupModalProps {
  isOpen: boolean,
  onOpenChange: () => void,
}

export function CreateGroupModal(props: CreateGroupModalProps) {
  const lastResult = useActionData<typeof action>();
  const [form, fields] = useForm({
		lastResult,
		constraint: getZodConstraint(CreateGroupMutationSchema),
	});

  return (
    <Modal isOpen={props.isOpen} onOpenChange={props.onOpenChange}>
      <ModalContent>
        {(onClose) => (
          <Form
            method="post"
            action={`/groups/create`}
            {...getFormProps(form)}
          >
            <ModalHeader className="flex flex-col">グループ作成</ModalHeader>
            <ModalBody>
              <Input
                name={fields.title.name}
                isRequired={fields.title.required}
                aria-invalid={fields.title.errors ? true : undefined}
                aria-describedby={fields.title.errors ? fields.title.errorId : undefined}
                label="タイトル"
                placeholder="例）ディズニーランド"
                variant="bordered"
              />
            </ModalBody>
            <ModalFooter>
              <Button color="danger" variant="flat" onPress={onClose}>
                キャンセル
              </Button>
              <Button color="primary" type="submit">
                作成
              </Button>
            </ModalFooter>
          </Form>
        )}
      </ModalContent>
    </Modal>
  )
}
