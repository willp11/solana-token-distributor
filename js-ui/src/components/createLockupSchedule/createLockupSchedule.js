import './createLockupSchedule.css';

const CreateLockupSchedule = () => {

    return (
        <div className="CreateLockupSchedule">
            <h2>Create Lockup Schedule</h2>
            <input placeholder="start timestamp (seconds)"/> <br/>
            <input placeholder="total unlock periods"/> <br/>
            <input placeholder="period duration (seconds)" /> <br/>
            <input placeholder="total lockup quantity" /> <br/>
            <button>Send tx</button>
        </div>
    );
}

export default CreateLockupSchedule;