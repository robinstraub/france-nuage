import { faker } from "@faker-js/faker";
import type { Organization } from "../models";

export const organization = (seed?: Partial<Organization>): Organization => ({
  id: faker.string.uuid(),
  name: faker.company.name(),
  ...seed,
});
