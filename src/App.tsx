import "./App.css";
import TabSwitcher from "./TabSwitcher";

const App = () => {
  return (
    <main className="container">
      <h1>Multi-Window App</h1>
      <p>
        Use the tabs below to switch between the Essay App (with Google authentication) and Perplexity AI.
      </p>
      {/* Renders the TabSwitcher to toggle between app windows */}
      <TabSwitcher />
    </main>
  );
};

export default App;
