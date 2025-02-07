const ReservationSidebar = () => {
    return (
        <aside className="mt-6 p-6 col-span-2 rounded-xl border border-gray-300">
            <h2 className="mb-5 text-2xl">
                $200 Per night
            </h2>
            <div className="mb-6 p-3 border border-gray-400 rounded-xl">
                <label className="mb-2 block font-bold text-xs">Guests</label>
                <select className="w-full ml-2 text-xm">
                    <option value="1">1 </option>
                    <option value="1">2 </option>
                    <option value="1">3 </option>
                    <option value="1">3 </option>
                    <option value="1">4 </option>


                </select>
            </div>
            <div className="w-full mb-6 py-6 text-center text-white bg-airbnb hover:bg-airbnb-dark rounded-xl"> 
            Book
            </div>
            <div className="mb-4 flex justify-between align-center">
                <p>$200 * 4 night</p>
                <p>$800</p>

            </div>
            <div className="mb-4 flex justify-between align-center">
                <p>Rentms Fee</p>
                <p>$40</p>

            </div>
            <hr />
            <div className="mt-4 flex justify-between align-center font-bold">
                <p>Total Fee</p>
                <p>$840</p>
            </div>
        
        </aside>
    )
}

export default ReservationSidebar;