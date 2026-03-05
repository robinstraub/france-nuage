import { Menu, Portal, Text } from "@chakra-ui/react";
import { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import {
  fetchOrganizations,
  setCurrent,
} from "../features/organizations/organizations-slice";
import type { AppDispatch, RootState } from "../store";

export function OrganizationSwitcher() {
  const dispatch = useDispatch<AppDispatch>();
  const { items, current } = useSelector(
    (state: RootState) => state.organizations,
  );

  useEffect(() => {
    dispatch(fetchOrganizations());
  }, [dispatch]);

  if (!current) {
    return null;
  }

  const handleSelect = (details: { value: string }) => {
    const selected = items.find((org) => org.id === details.value);
    if (selected) {
      dispatch(setCurrent(selected));
    }
  };

  return (
    <Menu.Root
      positioning={{ placement: "bottom-start" }}
      onSelect={handleSelect}
    >
      <Menu.Trigger
        borderWidth={1}
        alignItems="center"
        display="flex"
        gap="2"
        focusVisibleRing="outside"
        rounded="l2"
        p={1}
        height={8}
        maxWidth={{ base: 40, md: "full" }}
      >
        <Text
          fontWeight="medium"
          fontSize="sm"
          ms="1"
          overflow="hidden"
          textOverflow="ellipsis"
          textWrap="nowrap"
        >
          {current.name}
        </Text>
        <Text color="fg.muted" fontSize="xs">
          ▾
        </Text>
      </Menu.Trigger>
      <Portal>
        <Menu.Positioner>
          <Menu.Content minW={64}>
            {items.map((org) => (
              <Menu.Item key={org.id} value={org.id}>
                {org.name}
              </Menu.Item>
            ))}
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu.Root>
  );
}
