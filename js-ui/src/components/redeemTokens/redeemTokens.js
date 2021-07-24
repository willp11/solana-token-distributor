import './redeemTokens.css';

const RedeemTokens = () => {
    return (
        <div className="RedeemTokens">
            <h2>Redeem Tokens</h2>
            <input placeholder="receiving token public key"/> <br />
            <input placeholder="lockup schedule state public key" /> <br />
            <input placeholder="lockup state public key" /> <br />
            <input placeholder="lockup token account public key" /> <br />
            <button>Send tx</button>
        </div>
    );
}

export default RedeemTokens;