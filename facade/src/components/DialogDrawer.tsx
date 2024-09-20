"use client";
import { Dispatch, PropsWithChildren, SetStateAction } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
} from "./ui/dialog";
import {
  Drawer,
  DrawerContent,
  DrawerDescription,
  DrawerTitle,
  DrawerTrigger,
} from "./ui/drawer";
import { useMediaQuery } from "@/lib/useMediaQuery";

const DialogDrawer = ({
  children,
  open,
  setOpen,
  ...props
}: PropsWithChildren<{
  trigger?: JSX.Element;
  contentTitle: string;
  contentDescr?: string;
  open: boolean;
  setOpen: Dispatch<SetStateAction<boolean>>;
}>) => {
  const isDesktop = useMediaQuery("(min-width: 768px)");

  return isDesktop ? (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{props.trigger}</DialogTrigger>
      <DialogContent>
        <DialogTitle>{props.contentTitle}</DialogTitle>
        {props.contentDescr && (
          <DialogDescription>{props.contentDescr}</DialogDescription>
        )}
        <div className="mx-auto">{children}</div>
      </DialogContent>
    </Dialog>
  ) : (
    <Drawer open={open} onOpenChange={setOpen}>
      <DrawerTrigger asChild>{props.trigger}</DrawerTrigger>
      <DrawerContent>
        <DrawerTitle className="text-center pt-8">
          {props.contentTitle}
        </DrawerTitle>
        <DrawerDescription className="px-4 py-2 text-balance text-center">
          {props.contentDescr}
        </DrawerDescription>
        <div className="mx-auto">{children}</div>
      </DrawerContent>
    </Drawer>
  );
};

export { DialogDrawer };
