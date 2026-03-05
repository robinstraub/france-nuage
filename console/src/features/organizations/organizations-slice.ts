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
}

const initialState: OrganizationsState = {
  items: [],
  current: null,
  loading: false,
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
      })
      .addCase(fetchOrganizations.fulfilled, (state, action) => {
        state.items = action.payload;
        state.loading = false;
        if (!state.current && action.payload.length > 0) {
          state.current = action.payload[0];
        }
      })
      .addCase(fetchOrganizations.rejected, (state) => {
        state.loading = false;
      })
      .addCase(createOrganization.fulfilled, (state, action) => {
        state.items.push(action.payload);
        state.current = action.payload;
      });
  },
});

export const { setCurrent } = organizationsSlice.actions;
export default organizationsSlice.reducer;
