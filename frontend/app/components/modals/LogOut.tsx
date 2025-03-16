'use client';

import { resetAuthCookies } from "@/app/lib/actions";
import { useRouter } from "next/navigation";

const logOutModal = () => {
    resetAuthCookies();
    const router = useRouter();
    router.push('/')

}

export default logOutModal;