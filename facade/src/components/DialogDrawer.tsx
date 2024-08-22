import { PropsWithChildren, useState } from "react";
import { Button } from "./ui/button";
import { Dialog, DialogContent, DialogTitle, DialogTrigger } from "./ui/dialog";
import {
  Drawer,
  DrawerContent,
  DrawerHeader,
  DrawerTrigger,
} from "./ui/drawer";
import { useMediaQuery } from "@uidotdev/usehooks";

const DialogDrawer = ({
  children,
  ...props
}: PropsWithChildren<{
  // buttonVariant: keyof typeof buttonVariants;
  buttonText: string;
  contentHeaderText: string;
}>) => {
  const [open, setOpen] = useState<boolean>(false);
  const isDesktop = useMediaQuery("(min-width: 768px)");

  return isDesktop ? (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant={"outline"}>{props.buttonText}</Button>
      </DialogTrigger>
      <DialogContent className="max-w-xl">
        <DialogTitle>
          <h2>{props.contentHeaderText}</h2>
        </DialogTitle>
        {children}
      </DialogContent>
    </Dialog>
  ) : (
    <Drawer open={open} onOpenChange={setOpen}>
      <DrawerTrigger asChild>
        <Button variant={"outline"}>{props.buttonText}</Button>
      </DrawerTrigger>
      <DrawerContent className="max-w-xl">
        <DrawerHeader>
          <h2>{props.contentHeaderText}</h2>
        </DrawerHeader>
        {children}
      </DrawerContent>
    </Drawer>
  );
};

export { DialogDrawer };
