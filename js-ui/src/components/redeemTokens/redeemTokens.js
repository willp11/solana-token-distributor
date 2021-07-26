import './redeemTokens.css';
import { useSelector } from "react-redux";
import { useState } from 'react';
import { redeemTokens } from '../../util/redeemTokens';

const RedeemTokens = () => {
    const reduxState = useSelector(state => state);

    const [formData, setFormData] = useState({
        receivingTokenPubkey: null,
        lockupScheduleStatePubkey: null,
        lockupStatePubkey: null,
        lockupTokenAccountPubkey: null
    });

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
            const redeemInfo = await redeemTokens(
                reduxState.program_id,
                reduxState.wallet,
                formData.receivingTokenPubkey,
                formData.lockupScheduleStatePubkey,
                formData.lockupStatePubkey,
                formData.lockupTokenAccountPubkey
            );
            console.log(redeemInfo);
        }
    }

    return (
        <div className="RedeemTokens">
            <h2>Redeem Tokens</h2>
            <input name="receivingTokenPubkey" placeholder="receiving token public key" onChange={e=>inputHandler(e)}/> <br />
            <input name="lockupScheduleStatePubkey" placeholder="lockup schedule state public key" onChange={e=>inputHandler(e)}/> <br />
            <input name="lockupStatePubkey" placeholder="lockup state public key" onChange={e=>inputHandler(e)}/> <br />
            <input name="lockupTokenAccountPubkey" placeholder="lockup token account public key" onChange={e=>inputHandler(e)}/> <br />
            <button onClick={submitHandler}>Send tx</button>
        </div>
    );
}

export default RedeemTokens;