import './App.css';
import {
  BrowserRouter as Router,
  Switch,
  Route
} from "react-router-dom";
import Home from './components/home/home';
import CreateLockupSchedule from './components/createLockupSchedule/createLockupSchedule';
import LockTokens from './components/lockTokens/lockTokens';
import RedeemTokens from './components/redeemTokens/redeemTokens';

function App() {
  return (
    <div className="App">
      <h1>Solana Token Distributor</h1>
      <Router>
        <Home />
        <Switch>
          <Route path="/createLockupSchedule">
              <CreateLockupSchedule />
          </Route>
          <Route path="/lockTokens">
              <LockTokens />
          </Route>
          <Route path="/redeemTokens">
              <RedeemTokens />
          </Route>
          <Route path="/">
              <Home />
          </Route>
        </Switch>
      </Router>
    </div>
  );
}

export default App;
