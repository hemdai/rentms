import Image from "next/image";
import Link from "next/link";

const MyReservationsPage = () => {

    return (
        <main className="max-w-[1500px] mx-auto px-6 pb-6">
            <h1 className="my-6 text-2xl">My reservations</h1>

            <div className="space-y-4">            
                        <div className="p-5 grid grid-cols-1 md:grid-cols-4 gap-4 shadow-md border border-gray-300 rounded-xl">
                            <div className="col-span-1">
                                <div className="relative overflow-hidden aspect-square rounded-xl">
                                    <Image
                                        fill
                                        src="/beach_1.jpg"
                                        className="hover:scale-110 object-cover transition h-full w-full"
                                        alt="Beach house"
                                    />
                                </div>
                            </div>

                            <div className="col-span-1 md:col-span-3">
                                <h2 className="mb-4 text-xl">Reserved Property Title</h2>

                                <p className="mb-2"><strong>Check in date:</strong> Reservation Start Date</p>
                                <p className="mb-2"><strong>Check out date:</strong> Reservation End Date</p>

                                <p className="mb-2"><strong>Number of nights:</strong> Number of Nights</p>
                                <p className="mb-2"><strong>Total price:</strong> Total Price $280</p>

                                <Link 
                                    href=""
                                    className="mt-6 inline-block cursor-pointer py-4 px-6 bg-airbnb text-white rounded-xl"
                                >
                                    Go to property
                                </Link>
                            </div>
                        </div>
            </div>

        </main>
    )
}

export default MyReservationsPage;