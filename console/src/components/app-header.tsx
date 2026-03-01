import { Avatar, Box, Button, Flex, Text } from "@chakra-ui/react";
import type { User } from "oidc-client-ts";
import { userManager } from "../auth/user-manager";

interface AppHeaderProps {
  user: User;
  onToggleSidebar?: () => void;
}

export function AppHeader({ user, onToggleSidebar }: AppHeaderProps) {
  const displayName =
    user.profile.name ||
    user.profile.preferred_username ||
    user.profile.email ||
    "";

  const handleLogout = () => {
    userManager.signoutRedirect();
  };

  return (
    <Box as="header" borderBottomWidth="1px" borderColor="border" px={4}>
      <Flex h="16" align="center" justify="space-between">
        <Flex align="center" gap={3}>
          <Button
            variant="ghost"
            display={{ base: "inline-flex", lg: "none" }}
            onClick={onToggleSidebar}
            aria-label="Ouvrir le menu"
          >
            &#9776;
          </Button>
          <Text fontSize="lg" fontWeight="bold">
            France-nuage
          </Text>
        </Flex>
        <Flex align="center" gap={3}>
          <Text
            fontSize="sm"
            color="fg.muted"
            display={{ base: "none", md: "block" }}
          >
            {displayName}
          </Text>
          <Avatar.Root size="sm">
            <Avatar.Fallback name={displayName} />
          </Avatar.Root>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleLogout}
            aria-label="Se déconnecter"
          >
            Déconnexion
          </Button>
        </Flex>
      </Flex>
    </Box>
  );
}
