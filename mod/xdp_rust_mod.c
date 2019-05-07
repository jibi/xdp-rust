#include <linux/init.h>
#include <linux/module.h>
#include <linux/moduleparam.h>
#include <linux/kernel.h>
#include <linux/netdevice.h>

#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/tcp.h>

#include <net/xdp.h>

static char *iface = "enp1s0";
module_param(iface, charp, 0440);

static u32 xdp_rust_prog(struct xdp_buff *xdp)
{
	struct ethhdr *eth;
	struct iphdr  *ip;
	struct tcphdr *tcp;

	eth = (struct ethhdr*)xdp->data;
	if (eth + 1 > (struct ethhdr *)xdp->data_end)
		return XDP_ABORTED;
	if (eth->h_proto != htons(ETH_P_IP))
		return XDP_PASS;

	ip = (struct iphdr *)(eth + 1);
	if (ip + 1 > (struct iphdr *)xdp->data_end)
		return XDP_ABORTED;
	if (ip->daddr != htonl(0xc0a87ac1))
		return XDP_PASS;

	tcp = (struct tcphdr*)((u8 *)ip + ip->ihl * 4);
	if (tcp + 1 > (struct tcphdr *)xdp->data_end)
		return XDP_ABORTED;
	if (tcp->dest != htons(1234))
		return XDP_PASS;

	return XDP_DROP;
}

static int __init xdp_rust_init(void)
{
	struct net_device *dev;

	dev = dev_get_by_name(&init_net, iface);
	if (!dev)
		return 1;

	generic_xdp_rust_install(dev, xdp_rust_prog);

	return 0;
}

static void __exit xdp_rust_exit(void)
{
	struct net_device *dev;

	dev = dev_get_by_name(&init_net, iface);
	if (!dev)
		return;

	generic_xdp_rust_install(dev, NULL);
}

module_init(xdp_rust_init);
module_exit(xdp_rust_exit);

MODULE_LICENSE("GPL");
