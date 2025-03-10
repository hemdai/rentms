import Conversation from "../components/inbox/Conversation";

const MyInboxPage = () => {

    return (
        <main className="max-w-[1500px] mx-auto px-6 pb-6">
            <h1 className="my-6 text-2xl">InBox</h1>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <Conversation />
                <Conversation />

                <Conversation />
                
                <Conversation />


            </div>
        </main>
    )
}

export default MyInboxPage;