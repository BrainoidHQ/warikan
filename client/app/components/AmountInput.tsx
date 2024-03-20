import { type SerializeFrom } from "@vercel/remix";
import { useDebounceSubmit } from "remix-utils/use-debounce-submit";

import { type ControlButtonProps } from "@conform-to/dom"
import { FieldMetadata } from "@conform-to/react";

import {
  Button,
  Input,
  Select,
  SelectItem,
} from "@nextui-org/react";
import { TrashIcon } from '@radix-ui/react-icons'

import { GetGroupDetailQuery } from "~/gql/graphql";

// TODO: improve type annotations
export interface AmountInputProps {
  amount: FieldMetadata<
    {
      user: string;
      amount: number;
    },
    {
      title: string;
      creditors: {
          user: string;
          amount: number;
      }[];
      debtors: {
          user: string;
          amount: number;
      }[];
    },
    string[]
  >,
  participants: SerializeFrom<NonNullable<GetGroupDetailQuery["group"]>["participants"]>,
  removeButton: ControlButtonProps,
}

export function AmountInput(props: AmountInputProps) {
  const submit = useDebounceSubmit();
  const amount = props.amount.getFieldset();

  return (
    <li className="flex flex-row gap-2">
      <Select
        name={amount.user.name}
        aria-invalid={amount.user.errors ? true : undefined}
        aria-describedby={amount.user.errors ? amount.user.errorId : undefined}
        onChange={(event) => {
          submit(event.target.form, {
            navigate: false,
            fetcherKey: amount.user.id,
            debounceTimeout: 1000,
          });
        }}
        onBlur={(event) => {
          submit((event.target as HTMLInputElement).form, {
            navigate: false,
            fetcherKey: amount.user.id,
            debounceTimeout: 0,
          });
        }}
        defaultSelectedKeys={amount.user.initialValue ? [amount.user.initialValue] : undefined}
      >
        {props.participants.map((user) => (
          <SelectItem key={user.id} value={user.name}>
            {user.name}
          </SelectItem>
        ))}
      </Select>

      <Input
        type="number"
        name={amount.amount.name}
        aria-invalid={amount.amount.errors ? true : undefined}
        aria-describedby={amount.amount.errors ? amount.amount.errorId : undefined}
        onChange={(event) => {
          submit(event.target.form, {
            navigate: false,
            fetcherKey: amount.amount.id,
            debounceTimeout: 1000,
          });
        }}
        onBlur={(event) => {
          submit((event.target as HTMLInputElement).form, {
            navigate: false,
            fetcherKey: amount.amount.id,
            debounceTimeout: 0,
          });
        }}
        defaultValue={amount.amount.initialValue}
        startContent={
          <div className="pointer-events-none flex items-center">
            <span className="text-default-400 text-small">¥</span>
          </div>
        }
      />

      <Button
        {...props.removeButton}
        type="submit"
        isIconOnly
        size="sm"
        variant="bordered"
        color="danger"
      >
        <TrashIcon />
      </Button>
    </li>
  )
}
