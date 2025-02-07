import Image from "next/image";
import ReservationSidebar from "@/app/components/properties/ReservationSidebar";
const PropertyDetailPage = () => {
    return (
        <main className="max-w-[1500px] mx-auto px-6 pb-6">
            <div className="w-full h-[64vh] mv-4 overflow-hidden rounded-xl relative">
                <Image 
                    fill
                    src="/beach_1.jpg"
                    alt="Beach house"
                    sizes="(max-width: 768px) 768px, (max-width: 1200px) 768px, 768px"
                    className="object-cover h-full w-full"
                /> 

            </div>

            <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
                <div className="py-6 pr-6 col-span-3">
                    <h1 className="mb-4 text-4xl">Property Name</h1>
                    <span className="mb-6 block text-lg text-gray-600">
                        4 Guests- 2 Beds- 2 Baths
                    </span>
                    <hr />
                    <div className="py-6 flex items-center space-x-4">
                        <Image
                            src="/profile_pic_1.jpg"
                            alt="The user Name"
                            width={50}
                            height={50}
                            className="rounded-full"
                        />
                        <p> <strong>Shiva Shambhoo </strong>is your host</p>
                    </div>
                    <hr />
                    <p className="mt-6 text-lg">
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                        Quisquam, quod. Voluptate, repellat. 
                        Quisquam, quod. Voluptate, repellat.
                        Quisquam, quod. Voluptate, repellat.
                        Quisquam, quod. Voluptate, repellat.
                        Quisquam, quod. Voluptate, repellat.
                    </p>
                </div>
                <div className="col-span-2">
                    <ReservationSidebar />
                </div>

            </div>
            
    </main>
       
    )
}

export default PropertyDetailPage;