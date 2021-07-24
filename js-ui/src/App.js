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
import Wallet from '@project-serum/sol-wallet-adapter';
import { useDispatch, useSelector } from "react-redux";

function App() {

  const dispatch = useDispatch();
  const reduxState = useSelector(state => state);

  const startWallet = async () => {
      let providerUrl = 'https://www.sollet.io';
      let wallet = new Wallet(providerUrl);
  
      wallet.on('connect', publicKey => {
          console.log('Connected to ' + publicKey.toBase58());
      });
      wallet.on('disconnect', () => {
          console.log('Disconnected');
          dispatch({type: "walletDisconnected"});
      });
      await wallet.connect();
  
      return wallet;
  }

  const onStartWallet = async () => {
      let wallet = await startWallet();
      dispatch({
          type: "startWallet", 
          wallet: wallet
      });
  }

  return (
    <div className="App">
      <h1>Solana Token Distributor</h1>
      <button className="connect-btn" onClick={onStartWallet}>{reduxState.walletBtnTxt}</button>
      <Router>
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
