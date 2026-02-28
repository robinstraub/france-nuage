import { Provider } from "react-redux";
import { store } from "../store";

export function ReactReduxProvider(props: { children: React.ReactNode }) {
  return <Provider store={store}>{props.children}</Provider>;
}
