import { Box, Button, Flex, Text } from "@chakra-ui/react";
import { useLocation, useNavigate } from "react-router-dom";

interface SidebarItem {
  label: string;
  path: string;
}

interface SidebarSection {
  title: string;
  items: SidebarItem[];
}

const sections: SidebarSection[] = [
  {
    title: "Applications",
    items: [
      { label: "Mes applications", path: "/" },
      { label: "Catalogue", path: "/catalog" },
    ],
  },
  {
    title: "Organisation",
    items: [{ label: "Facturation", path: "/billing" }],
  },
];

export function AppSidebar() {
  const location = useLocation();
  const navigate = useNavigate();

  return (
    <Box as="nav" aria-label="Navigation principale" p={4}>
      <Flex direction="column" gap={6}>
        {sections.map((section) => (
          <Flex key={section.title} direction="column" gap={1}>
            <Text
              fontSize="xs"
              fontWeight="medium"
              textTransform="uppercase"
              color="fg.muted"
              px={3}
              mb={1}
            >
              {section.title}
            </Text>
            {section.items.map((item) => {
              const isActive = location.pathname === item.path;
              return (
                <Button
                  key={item.path}
                  variant="ghost"
                  justifyContent="flex-start"
                  fontWeight={isActive ? "medium" : "normal"}
                  bg={isActive ? "colorPalette.subtle" : undefined}
                  borderLeftWidth={isActive ? "3px" : "3px"}
                  borderLeftColor={isActive ? "colorPalette.fg" : "transparent"}
                  borderRadius="md"
                  aria-current={isActive ? "page" : undefined}
                  onClick={() => navigate(item.path)}
                >
                  {item.label}
                </Button>
              );
            })}
          </Flex>
        ))}
      </Flex>
    </Box>
  );
}
