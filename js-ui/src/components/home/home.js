import './home.css';
import {Link} from 'react-router-dom';

const Home = () => {
    return (
        <div className="Home">
            <ul>
                <li><Link to="/createLockupSchedule">Create Lockup Schedule</Link></li>
                <li><Link to="/lockTokens">Lock Tokens</Link></li>
                <li><Link to="/redeemTokens">Redeem Tokens</Link></li>
            </ul>
        </div>
    );
}

export default Home;