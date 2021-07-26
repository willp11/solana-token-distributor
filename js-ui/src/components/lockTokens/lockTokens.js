import './lockTokens.css';
import { useSelector } from "react-redux";
import { useState } from 'react';
import { lockTokens } from '../../util/lockTokens';

const LockTokens = () => {

    const reduxState = useSelector(state => state);

    const [formData, setFormData] = useState({
        lockupScheduleStatePubkey: null,
        tokenMint: null,
        senderPubkey: null,
        receiverPubkey: null,
        quantity: null
    })

    const inputHandler = (e) => {
        let newValue = e.target.value;
        let state = {...formData};
        state[e.target.name] = newValue;
        setFormData(state);
    }

    const submitHandler = async () => {
        if (reduxState.wallet == null) {
            alert("Connect to wallet");
        } else {
            const lockupInfo = await lockTokens(
                reduxState.program_id,
                reduxState.wallet,
                formData.lockupScheduleStatePubkey,
                formData.tokenMint,
                formData.senderPubkey,
                formData.receiverPubkey,
                formData.quantity
            );
            console.log(lockupInfo);
        }
    }

    return (
        <div className="LockTokens">
            <h2>Lock Tokens</h2>
            <input name="lockupScheduleStatePubkey" placeholder="lockup-schedule state public key" onChange={e=>inputHandler(e)}/> <br/>
            <input name="tokenMint" placeholder="token mint" onChange={e=>inputHandler(e)}/> <br/>
            <input name="senderPubkey" placeholder="sender token account public key" onChange={e=>inputHandler(e)}/> <br/>
            <input name="receiverPubkey" placeholder="receiver SOL account public key" onChange={e=>inputHandler(e)}/> <br/>
            <input name="quantity" placeholder="quantity of tokens" onChange={e=>inputHandler(e)}/> <br/>
            <button onClick={submitHandler}>Send Tx</button>
        </div>
    );
}

export default LockTokens;