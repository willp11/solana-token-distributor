import './createLockupSchedule.css';
import { useSelector } from "react-redux";
import { useState } from 'react';
import { createLockupSchedule } from '../../util/createLockupSchedule';

const CreateLockupSchedule = () => {

    const reduxState = useSelector(state => state);

    const [formData, setFormData] = useState({
        startTimestamp: null,
        totalUnlockPeriods: null,
        periodDuration: null,
        totalLockupQuantity: null,
        tokenMintPubkey: null
    })

    const inputHandler = (e) => {
        let state = {...formData};
        state[e.target.name] = e.target.value;
        setFormData(state);
    }

    const submitHandler = async () => {
        if (reduxState.wallet == null) {
            alert("Connect to wallet");
        } else {
            console.log("Sending tx...");
            const lockupSchedule = await createLockupSchedule(
                reduxState.program_id,
                reduxState.wallet,
                formData.startTimestamp,
                formData.totalUnlockPeriods,
                formData.periodDuration,
                formData.totalLockupQuantity,
                formData.tokenMintPubkey
            );
            console.log(lockupSchedule);
        }
    }
    
    return (
        <div className="CreateLockupSchedule">
            <h2>Create Lockup Schedule</h2>
            <input type="number" name="startTimestamp" placeholder="start timestamp (seconds)" onChange={e=>inputHandler(e)}/> <br/>
            <input type="number" name="totalUnlockPeriods" placeholder="total unlock periods"/> <br/>
            <input type="number" name="periodDuration" placeholder="period duration (seconds)" /> <br/>
            <input type="number" name="totalLockupQuantity" placeholder="total lockup quantity" /> <br/>
            <input type="text" name="tokenMintPubkey" placeholder="token mint public key (base58)" /> <br/>
            <button onClick={submitHandler}>Send tx</button>
        </div>
    );
}

export default CreateLockupSchedule;