#include <linux/init.h>
#include <linux/module.h>
#include <linux/moduleparam.h>
#include <linux/kernel.h>
#include <linux/netdevice.h>

#include <net/xdp.h>

static char *iface = "enp1s0";
module_param(iface, charp, 0440);

extern u32 xdp_rust_prog(struct xdp_buff *xdp);

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
