import { ReactNode } from "react";
import { type PressEvent } from "@react-types/shared";

import {
  BreadcrumbItem,
  Breadcrumbs,
  Button,
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
} from "@nextui-org/react";

export interface AppBarBreadcrumbItem {
  label: string;
  href: string;
}

export interface AppBarButtonItem {
  label: string;
  icon: ReactNode;
  onPress: (e: PressEvent) => void;
}

export interface AppBarProps {
  breadcrumbs: AppBarBreadcrumbItem[];
  buttons: AppBarButtonItem[];
}

export function AppBar(props: AppBarProps) {
  return (
    <Navbar position="static">
      <NavbarBrand>
        <Breadcrumbs size="lg">
          {props.breadcrumbs.map((item) => (
            <BreadcrumbItem key={item.label} href={item.href}>
              {item.label}
            </BreadcrumbItem>
          ))}
        </Breadcrumbs>
      </NavbarBrand>
      <NavbarContent justify="end">
        {props.buttons.map((item) => (
          <NavbarItem key={item.label}>
            <Button isIconOnly variant="flat" onPress={item.onPress}>
              {item.icon}
            </Button>
          </NavbarItem>
        ))}
      </NavbarContent>
    </Navbar>
  );
}
