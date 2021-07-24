import './lockTokens.css';

const LockTokens = () => {

    return (
        <div className="LockTokens">
            <h2>Lock Tokens</h2>
            <input placeholder="lockup-schedule state public key"/> <br/>
            <input placeholder="receiver public key"/> <br/>
            <input placeholder="number of tokens" /> <br/>
            <button>Send Tx</button>
        </div>
    );
}

export default LockTokens;