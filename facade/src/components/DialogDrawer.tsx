import { PropsWithChildren, useState } from "react";
import { Button } from "./ui/button";
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
import { useMediaQuery } from "@uidotdev/usehooks";

const DialogDrawer = ({
  children,
  ...props
}: PropsWithChildren<{
  // buttonVariant: keyof typeof buttonVariants;
  buttonText: string;
  contentTitle: string;
  contentDescr: string;
}>) => {
  const [open, setOpen] = useState<boolean>(false);
  const isDesktop = useMediaQuery("(min-width: 768px)");

  return isDesktop ? (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant={"outline"}>{props.buttonText}</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>{props.contentTitle}</DialogTitle>
        <DialogDescription>{props.contentDescr}</DialogDescription>
        <div className="mx-auto">{children}</div>
      </DialogContent>
    </Dialog>
  ) : (
    <Drawer open={open} onOpenChange={setOpen}>
      <DrawerTrigger asChild>
        <Button variant={"outline"}>{props.buttonText}</Button>
      </DrawerTrigger>
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
