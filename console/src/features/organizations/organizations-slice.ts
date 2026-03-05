import {
  type Organization,
  ResourceManagerServiceClient,
} from "@france-nuage/node-sdk";
import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { getAuthenticatedTransport } from "../../services/grpc-transport";

export const fetchOrganizations = createAsyncThunk(
  "organizations/fetch",
  async () => {
    const transport = await getAuthenticatedTransport();
    const client = new ResourceManagerServiceClient(transport);
    const { response } = await client.listOrganizations({});
    return response.organizations.map((org) => ({
      id: org.id,
      name: org.name,
    }));
  },
);

export const createOrganization = createAsyncThunk(
  "organizations/create",
  async (name: string) => {
    const transport = await getAuthenticatedTransport();
    const client = new ResourceManagerServiceClient(transport);
    const { response } = await client.createOrganization({ name });
    const org = response.organization;
    if (!org) throw new Error("organization missing from response");
    return { id: org.id, name: org.name };
  },
);

interface OrganizationsState {
  items: Organization[];
  current: Organization | null;
  loading: boolean;
  status: "idle" | "loading" | "succeeded" | "failed";
  error: string | null;
}

const initialState: OrganizationsState = {
  items: [],
  current: null,
  loading: false,
  status: "idle",
  error: null,
};

const organizationsSlice = createSlice({
  name: "organizations",
  initialState,
  reducers: {
    setCurrent(state, action: { payload: Organization }) {
      state.current = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchOrganizations.pending, (state) => {
        state.loading = true;
        state.status = "loading";
        state.error = null;
      })
      .addCase(fetchOrganizations.fulfilled, (state, action) => {
        state.items = action.payload;
        state.loading = false;
        state.status = "succeeded";
        if (!state.current && action.payload.length > 0) {
          state.current = action.payload[0];
        }
      })
      .addCase(fetchOrganizations.rejected, (state, action) => {
        state.loading = false;
        state.status = "failed";
        state.error =
          action.error.message ?? "Erreur lors du chargement des organisations";
      })
      .addCase(createOrganization.fulfilled, (state, action) => {
        state.items.push(action.payload);
        state.current = action.payload;
      })
      .addCase(createOrganization.rejected, (state, action) => {
        state.error =
          action.error.message ??
          "Erreur lors de la création de l'organisation";
      });
  },
});

export const { setCurrent } = organizationsSlice.actions;
export default organizationsSlice.reducer;
