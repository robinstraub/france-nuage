import { Box, Button, Flex, Heading, Input, Text } from "@chakra-ui/react";
import { useState } from "react";
import { useDispatch } from "react-redux";
import { useNavigate } from "react-router-dom";
import { createOrganization } from "../features/organizations/organizations-slice";
import type { AppDispatch } from "../store";

export function OnboardingPage() {
  const [name, setName] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const dispatch = useDispatch<AppDispatch>();
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      setError("Le nom de l'organisation est requis");
      return;
    }
    setLoading(true);
    setError("");
    try {
      await dispatch(createOrganization(name.trim())).unwrap();
      navigate("/");
    } catch {
      setError("Erreur lors de la création de l'organisation");
    } finally {
      setLoading(false);
    }
  };

  return (
    <Flex h="100vh" align="center" justify="center">
      <Box w="full" maxW="md" p={8}>
        <Heading size="lg" mb={2}>
          Bienvenue sur France-nuage
        </Heading>
        <Text color="fg.muted" mb={8}>
          Créez votre première organisation pour commencer.
        </Text>
        <form onSubmit={handleSubmit}>
          <Input
            placeholder="Nom de l'organisation"
            value={name}
            onChange={(e) => setName(e.target.value)}
            mb={4}
          />
          {error && (
            <Text color="red.500" fontSize="sm" mb={4}>
              {error}
            </Text>
          )}
          <Button type="submit" w="full" loading={loading}>
            Créer l'organisation
          </Button>
        </form>
      </Box>
    </Flex>
  );
}
