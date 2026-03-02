import {
  Box,
  DrawerBackdrop,
  DrawerBody,
  DrawerCloseTrigger,
  DrawerContent,
  DrawerRoot,
  Flex,
} from "@chakra-ui/react";
import type { User } from "oidc-client-ts";
import { useState } from "react";
import { Outlet, useOutletContext } from "react-router-dom";
import { AppHeader } from "./app-header";
import { AppSidebar } from "./app-sidebar";

export function AppLayout() {
  const user = useOutletContext<User>();
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <Flex direction="column" h="100vh">
      <AppHeader user={user} onToggleSidebar={() => setSidebarOpen(true)} />
      <Flex flex={1} overflow="hidden">
        <Box
          display={{ base: "none", lg: "block" }}
          w="280px"
          borderRightWidth="1px"
          borderColor="border"
          overflowY="auto"
        >
          <AppSidebar />
        </Box>

        <DrawerRoot
          open={sidebarOpen}
          onOpenChange={(e) => setSidebarOpen(e.open)}
          placement="start"
        >
          <DrawerBackdrop />
          <DrawerContent>
            <DrawerCloseTrigger />
            <DrawerBody p={0}>
              <AppSidebar />
            </DrawerBody>
          </DrawerContent>
        </DrawerRoot>

        <Box as="main" flex={1} overflowY="auto" p={4}>
          <Outlet />
        </Box>
      </Flex>
    </Flex>
  );
}
